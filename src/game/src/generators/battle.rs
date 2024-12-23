use super::Generator;
use aws_sdk_bedrockruntime::types::builders::*;
use llm::llm::LlmHandler;
use llm::llm::ParseConverseOuput;
use llm::tool::ToolError;
use llm::tool::{BattleToolOutput, Tools};
use std::any::type_name;

static BATTLE_GENERATOR_SYSTEM_MESSAGE: &str = "
You are a DM/Narrator for a single player dungeons and dragons style text adventure game.
It is important that you always create unique and fun scenarios with a wide variety of situations, enemies, items, and settings to keep the player engaged.
You are able to create 3 different scenario types. Battle, Shop, and Rest.
When creating scenarios keep them inline with the theme of the level.
You always keep your scenarios to single 2-4 sentence paragraphs, without bullet points.
";

static BATTLE_GENERATOR_TO_JSON_INSTRUCTIONS: &str = "
Using the provide context use the 'battle' tool to generate a json response.
";

static BATTLE_GENERATOR_PROMPT: &str = "
Create a random battle scenario for the player.
";

#[derive(Debug, Clone)]
pub struct BattleGenerator {
    model: LlmHandler,
    // TODO level and theme could probably be put into a "Scenario" struct
    level: u8,
    theme: String,
    pub context: Option<Vec<String>>,
    pub scenario: Option<String>,
    pub output: Option<BattleToolOutput>,
    tool: Tools,
}

impl BattleGenerator {
    pub async fn new(
        model: String,
        system: String,
        level: u8,
        theme: String,
        context: Option<Vec<String>>,
    ) -> anyhow::Result<Self> {
        // let system = BATTLE_GENERATOR_SYSTEM_MESSAGE.to_string();
        let model = LlmHandler::new(model, system, None).await;
        let scenario = None;
        let tool = Tools::Battle;
        let output = None;

        Ok(Self {
            model,
            level,
            theme,
            context,
            scenario,
            tool,
            output,
        })
    }

    pub async fn generate_scenario(&mut self, prompt: String) -> anyhow::Result<()> {
        // let prompt = format!(
        //     "Create me a battle scenario for a {} theme level.",
        //     self.theme
        // );

        let prompt = self.model.create_prompt(self.context.clone(), &prompt);

        let message = self.model.create_user_message(&prompt);

        self.model.set_messages(vec![message]);

        let inference_cfg_builder = InferenceConfigurationBuilder::default()
            .temperature(1.0)
            .top_p(1.0)
            .max_tokens(1000)
            .build();

        let inference_cfg = Some(inference_cfg_builder);

        let res = self.model.converse(None, inference_cfg).await?;

        let scenario = res.get_text_output()?;

        self.scenario = Some(scenario);

        Ok(())
    }

    pub async fn to_json(&mut self, prompt: String) -> anyhow::Result<()> {
        // THIS is a static prompt using lazy_static
        // let prompt = "".to_string();

        let scenario = self.scenario.clone().expect("Scenario not found");

        let context = Some(vec![scenario]);

        let input = self.model.create_prompt(context, &prompt);

        let message = self.model.create_user_message(&input);

        let inference_cfg_builder = InferenceConfigurationBuilder::default()
            .temperature(0.3)
            .top_p(0.8)
            .max_tokens(1000)
            .build();

        let inference_cfg = Some(inference_cfg_builder);

        self.model.set_messages(vec![message]);

        let res = self.model.converse(Some(self.tool), inference_cfg).await?;

        let tool_output = match res.get_tool_output() {
            Ok(tool) => {
                let tool_use_block = tool[0].input.clone();

                Ok(tool_use_block)
            }
            Err(_) => {
                let name = type_name::<BattleToolOutput>();

                let err = ToolError::ParseOutput(name);

                Err(err)
            }
        };

        let output = match tool_output {
            Ok(tool) => {
                let value = serde_json::to_value(tool)?;
                println!("TOOL VALUE: {:?}", value);

                let output: BattleToolOutput = serde_json::from_value(value.clone())?;

                Ok(output)
            }
            Err(_) => {
                let name = type_name::<BattleToolOutput>();

                let err = ToolError::ParseOutput(name);

                Err(err)
            }
        }?;

        self.output = Some(output);

        Ok(())
    }
}

impl Generator for BattleGenerator {
    fn generate(&self) -> String {
        "".to_string()
    }
}
