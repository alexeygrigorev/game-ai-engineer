//! Test binary for LLM integration
//!
//! Run with:
//!   cargo run --bin test_llm_integration
//!
//! Tests:
//! - Config loading
//! - NpcEngine with mock provider
//! - NpcEngine with real API (optional, requires ANTHROPIC_API_KEY)

use ai_career_rpg::engine::{GameConfig, GameContext, NpcEngine, NpcInput};
use ai_career_rpg::llm::{LlmConfig, LlmProvider, Provider, create_provider, MockProvider};

#[tokio::main]
async fn main() {
    println!("=== LLM Integration Test ===\n");

    // Test 1: Load game config
    println!("1. Loading game config...");
    let config = match GameConfig::load() {
        Ok(c) => {
            println!("   ✓ Config loaded successfully");
            println!("   - LLM provider: {}", c.llm.provider);
            println!("   - LLM model: {}", c.llm.model);
            println!("   - NPC default engine: {}", c.npc.default_engine);
            c
        }
        Err(e) => {
            eprintln!("   ✗ Failed to load config: {}", e);
            return;
        }
    };

    // Test 2: Create mock provider
    println!("\n2. Testing mock provider...");
    let mock = MockProvider::new("Hello, brave adventurer!");
    let result = mock.complete("You are an NPC", vec![ai_career_rpg::llm::LlmMessage::user("Hi")])
        .await
        .expect("Mock should not fail");
    println!("   ✓ Mock response: {}", result);

    // Test 3: Create NpcEngine with mock
    println!("\n3. Testing NpcEngine with mock provider...");
    let test_config = GameConfig::load().expect("Config should load");
    let mut engine = NpcEngine::with_mock(test_config, "I am a test NPC response!");
    
    let input = NpcInput {
        npc_id: 1,
        npc_class: "barista".to_string(),
        npc_name: "Test Barista".to_string(),
        player_message: None,
    };
    
    let context = GameContext::empty();
    
    match engine.get_dialog(&input, &context).await {
        Ok(output) => {
            println!("   ✓ NPC response: {}", output.text);
            println!("   - From LLM: {}", output.from_llm);
        }
        Err(e) => {
            eprintln!("   ✗ Failed: {}", e);
        }
    }

    // Test 4: Test LLM-powered NPC (recruiter has engine="llm" in config)
    println!("\n4. Testing LLM-powered NPC (recruiter) with mock...");
    let input = NpcInput {
        npc_id: 2,
        npc_class: "recruiter".to_string(),
        npc_name: "Alex".to_string(),
        player_message: Some("What jobs do you have?".to_string()),
    };
    
    let context = GameContext {
        player_name: "Test Player".to_string(),
        top_skills: vec![
            ai_career_rpg::engine::context::SkillInfo {
                name: "Python".to_string(),
                proficiency: "Intermediate".to_string(),
            },
        ],
        employed: false,
        current_job: None,
        day: 5,
    };
    
    match engine.get_dialog(&input, &context).await {
        Ok(output) => {
            println!("   ✓ Recruiter response: {}", output.text);
            println!("   - From LLM: {}", output.from_llm);
        }
        Err(e) => {
            eprintln!("   ✗ Failed: {}", e);
        }
    }

    // Test 5: Real API test (optional)
    println!("\n5. Testing real API (if credentials available)...");
    
    let provider_config = LlmConfig {
        provider: "anthropic".to_string(),
        model: "glm-4.7".to_string(),
    };
    
    match create_provider(&provider_config) {
        Ok(provider) => {
            println!("   Provider created, testing API call...");
            
            let system = "You are a helpful test assistant. Respond with exactly: 'API test successful!'";
            let messages = vec![ai_career_rpg::llm::LlmMessage::user("Test")];
            
            match provider.complete(system, messages).await {
                Ok(response) => {
                    println!("   ✓ Real API response: {}", response);
                }
                Err(e) => {
                    println!("   ✗ API call failed: {}", e);
                    println!("   (This is expected if no valid API key is configured)");
                }
            }
        }
        Err(e) => {
            println!("   ✗ Could not create provider: {}", e);
            println!("   (Set ANTHROPIC_API_KEY and ANTHROPIC_BASE_URL in .env for real API test)");
        }
    }

    println!("\n=== Test Complete ===");
}
