//! LLM Response Cache
//!
//! Caches LLM responses to reduce API calls and costs.
//! Uses context hashing to ensure relevant cache hits.
//!
//! # Cache Key
//! Tuple of (activity_type, input_hash, context_hash)
//! - `activity_type`: "npc_dialog", "interview", etc.
//! - `input_hash`: Hash of NPC class / prompt
//! - `context_hash`: Hash of player state (skills, day, etc.)
//!
//! # Cache Policy
//! - TTL: 5 minutes (configurable)
//! - Storage: In-memory, cleared on game exit
//! - Max entries: 100 per activity (LRU eviction)

use std::collections::HashMap;
use std::time::{Duration, Instant};

use super::context::GameContext;

/// LLM response cache entry
struct CacheEntry {
    /// Cached response text
    response: String,
    /// When this entry was created
    created_at: Instant,
}

impl CacheEntry {
    fn new(response: String) -> Self {
        Self {
            response,
            created_at: Instant::now(),
        }
    }

    fn is_expired(&self, ttl: Duration) -> bool {
        self.created_at.elapsed() > ttl
    }
}

/// LLM Response Cache
///
/// Thread-safe cache for storing LLM responses.
/// Uses LRU eviction when max entries is reached.
pub struct ResponseCache {
    /// Cached entries by key
    entries: HashMap<String, CacheEntry>,
    /// Order of access for LRU eviction
    access_order: Vec<String>,
    /// Time-to-live for cache entries
    ttl: Duration,
    /// Maximum entries before LRU eviction
    max_entries: usize,
}

impl ResponseCache {
    /// Create a new cache with default settings
    ///
    /// Default: 5 minute TTL, 100 max entries
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            access_order: Vec::new(),
            ttl: Duration::from_secs(300), // 5 minutes
            max_entries: 100,
        }
    }

    /// Create a cache with custom settings
    pub fn with_settings(ttl: Duration, max_entries: usize) -> Self {
        Self {
            entries: HashMap::new(),
            access_order: Vec::new(),
            ttl,
            max_entries,
        }
    }

    /// Generate a cache key from activity, input, and context
    ///
    /// The key is a combination of:
    /// - Activity type (e.g., "npc_recruiter")
    /// - Input identifier (e.g., conversation turn or question)
    /// - Context hash (relevant player state)
    pub fn make_key(activity: &str, input_id: &str, context: &GameContext) -> String {
        // Create a context hash from relevant fields
        // We only include fields that affect the response
        let context_str = format!(
            "{}|{:?}|{}|{}",
            context.player_name,
            context
                .top_skills
                .iter()
                .map(|s| &s.name)
                .collect::<Vec<_>>(),
            context.employed,
            context.day / 10, // Group by 10-day periods to allow some reuse
        );

        // Simple hash (good enough for caching)
        let context_hash = Self::simple_hash(&context_str);

        format!("{}|{}|{:08x}", activity, input_id, context_hash)
    }

    /// Simple string hash for cache keys
    fn simple_hash(s: &str) -> u32 {
        let mut hash: u32 = 0;
        for c in s.chars() {
            hash = hash.wrapping_mul(31).wrapping_add(c as u32);
        }
        hash
    }

    /// Get a cached response if valid
    ///
    /// Returns None if:
    /// - Key not in cache
    /// - Entry has expired
    pub fn get(&mut self, key: &str) -> Option<String> {
        // Check if entry exists and is not expired
        let expired = self
            .entries
            .get(key)
            .map(|e| e.is_expired(self.ttl))
            .unwrap_or(false);

        if expired {
            // Remove expired entry
            self.entries.remove(key);
            self.access_order.retain(|k| k != key);
            return None;
        }

        if let Some(entry) = self.entries.get(key) {
            // Update access order for LRU
            self.access_order.retain(|k| k != key);
            self.access_order.push(key.to_string());
            return Some(entry.response.clone());
        }

        None
    }

    /// Store a response in the cache
    pub fn set(&mut self, key: String, response: String) {
        // Evict LRU entries if at capacity
        while self.entries.len() >= self.max_entries {
            if let Some(lru_key) = self.access_order.first().cloned() {
                self.entries.remove(&lru_key);
                self.access_order.remove(0);
            } else {
                break;
            }
        }

        // Add new entry
        self.access_order.push(key.clone());
        self.entries.insert(key, CacheEntry::new(response));
    }

    /// Clear all cached entries
    pub fn clear(&mut self) {
        self.entries.clear();
        self.access_order.clear();
    }

    /// Get number of cached entries (for debugging)
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if cache is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl Default for ResponseCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_key_generation() {
        let ctx1 = GameContext {
            player_name: "Alice".to_string(),
            top_skills: vec![],
            employed: false,
            current_job: None,
            day: 5,
        };

        let ctx2 = GameContext {
            player_name: "Bob".to_string(),
            top_skills: vec![],
            employed: false,
            current_job: None,
            day: 5,
        };

        let key1 = ResponseCache::make_key("npc", "recruiter", &ctx1);
        let key2 = ResponseCache::make_key("npc", "recruiter", &ctx2);

        // Different players should have different keys
        assert_ne!(key1, key2);
    }

    #[test]
    fn test_cache_set_and_get() {
        let mut cache = ResponseCache::new();
        let ctx = GameContext::empty();
        let key = ResponseCache::make_key("test", "input", &ctx);

        cache.set(key.clone(), "response".to_string());

        assert_eq!(cache.get(&key), Some("response".to_string()));
    }

    #[test]
    fn test_cache_miss() {
        let mut cache = ResponseCache::new();
        assert_eq!(cache.get("nonexistent"), None);
    }

    #[test]
    fn test_cache_lru_eviction() {
        let mut cache = ResponseCache::with_settings(Duration::from_secs(3600), 3);
        let ctx = GameContext::empty();

        // Add 3 entries
        cache.set("key1".to_string(), "v1".to_string());
        cache.set("key2".to_string(), "v2".to_string());
        cache.set("key3".to_string(), "v3".to_string());

        // Access key1 to make it recently used
        cache.get("key1");

        // Add 4th entry - should evict key2 (LRU)
        cache.set("key4".to_string(), "v4".to_string());

        assert_eq!(cache.get("key1"), Some("v1".to_string())); // Still there
        assert_eq!(cache.get("key2"), None); // Evicted
        assert_eq!(cache.get("key3"), Some("v3".to_string())); // Still there
        assert_eq!(cache.get("key4"), Some("v4".to_string())); // New entry
    }
}
