use dice::Dice;
use serenity::builder::*;
use serenity::model::application::*;

pub struct Roll;
impl Roll {
    pub fn command(cmd: CommandInteraction) -> CreateInteractionResponse {
        let count = Self::get_count(&cmd);

        let sides = Self::get_sides(&cmd);

        let modifier = match &cmd.data.options.iter().find(|o| o.name == "modifier") {
            Some(m) => m
                .value
                .as_i64()
                .expect("could not convert modifier.value to integer")
                .try_into()
                .expect("could not convert to usize"),
            None => 0,
        };

        let dice: Vec<Dice> = (0..count).map(|_| Dice::new(sides)).collect();

        let dice_values: Vec<usize> = dice.into_iter().map(|d| d.roll()).collect::<Vec<usize>>();

        let roll: usize = dice_values.iter().sum();

        let content = format!(
            "Roll: {}\nYou rolled {}!",
            Self::create_roll_text(dice_values, modifier),
            roll
        );

        let message = CreateInteractionResponseMessage::new().content(content);

        CreateInteractionResponse::Message(message)
    }

    fn create_roll_text(dice_values: Vec<usize>, modifier: usize) -> String {
        if modifier > 0 {
            format!(
                "[{}{}] + {}",
                dice_values[0],
                dice_values[1..]
                    .iter()
                    .map(|v| format!(" + {}", v))
                    .collect::<Vec<String>>()
                    .join(""),
                modifier
            )
        } else {
            format!(
                "[{}{}]",
                dice_values[0],
                dice_values[1..]
                    .iter()
                    .map(|v| format!(" + {}", v))
                    .collect::<Vec<String>>()
                    .join("")
            )
        }
    }

    fn get_sides(cmd: &CommandInteraction) -> usize {
        let sides: usize = cmd
            .data
            .options
            .iter()
            .find(|o| o.name == "sides")
            .expect("Could not find 'sides' option for /roll")
            .value
            .as_i64()
            .expect("could not convert sides.value to integer")
            .try_into()
            .expect("could not convert to usize");
        sides
    }

    fn get_count(cmd: &CommandInteraction) -> usize {
        let count: usize = cmd
            .data
            .options
            .iter()
            .find(|o| o.name == "count")
            .expect("could not find 'count' option for /roll")
            .value
            .as_i64()
            .expect("could not convert count.value to integer")
            .try_into()
            .expect("could not convert to usize");
        count
    }
}
