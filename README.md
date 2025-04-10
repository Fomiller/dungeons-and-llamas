<p align="center">
    <img src="src/_data/DNL_README.jpg" alt="Dungeons and Llamas" width="600"/>
</p>

<!-- Badges (Uncomment the lines below for active badges) -->
<!-- [![crates.io](https://img.shields.io/crates/v/my_crate.svg)](https://crates.io/crates/my_crate) -->
<!-- [![docs.rs](https://docs.rs/my_crate/badge.svg)](https://docs.rs/my_crate) -->
<!-- [![Build Status](https://github.com/Fomiller/dungesons-and-llamas/actions/workflows/deploy.yml/badge.svg)](https://github.com/Fomiller/dungeons-and-llamas/actions) -->
<!-- [![codecov](https://codecov.io/gh/myusername/myrepo/branch/main/graph/badge.svg)](https://codecov.io/gh/myusername/myrepo) -->
![Rust Version](https://img.shields.io/badge/rustc-1.86+-orange.svg)
<!-- [![License](https://img.shields.io/crates/l/my_crate.svg)](https://crates.io/crates/my_crate) -->

# Dungeons and Llamas

## Description

Dungeons and Llamas is a serverless Discord bot written in Rust, leveraging AWS services such as API Gateway, DynamoDB, Lambda, Bedrock, and more! This bot creates dynamic, text-based adventures powered by LLMs, allowing users to experience unique, AI-generated stories each time they play.

The bot uses Discord's API as a UI, offering players a variety of commands throughout their adventure. 

### Key Features
- **Dynamic Story Generation**: AI creates content based on player inputs.
- **Discord Command Integration**: Players can interact with the game using Discord commands.

### Starting a Game

To start a new game, the user will use the `/new-game` command, where they will fill in some basic details about their character.  

<img src="https://github.com/Fomiller/dungeons-and-llamas/blob/develop/src/_data/new_game_cmd.png" width="50%" />

Players can specify a theme for their game, and the AI will generate content related to that theme.

### Adjusting Campaign Settings

If a player wants to adjust the settings of their campaign, they can use the `/settings` command.  
<img src="https://github.com/Fomiller/dungeons-and-llamas/blob/develop/src/_data/settings_cmd.png" width="50%" />

### Game Start

Once the game starts, players will choose from three different weapons to begin their adventure.  
<img src="https://github.com/Fomiller/dungeons-and-llamas/blob/develop/src/_data/new_game_gen.png" width="50%" />

### Generated Scenarios

The bot currently supports three different scenarios:

1. **Battle**  
<img src="https://github.com/Fomiller/dungeons-and-llamas/blob/develop/src/_data/battle_scenario_gen.png" width="50%" />

2. **Shop**  
<img src="https://github.com/Fomiller/dungeons-and-llamas/blob/develop/src/_data/shop_scenario_gen.png" width="50%" />

3. **Rest**  
<img src="https://github.com/Fomiller/dungeons-and-llamas/blob/develop/src/_data/rest_scenario_gen.png" width="50%" />

``## Installation

To add the Dungeons-and-Llamas bot to your Discord, follow this [link](https://discord.com/oauth2/authorize?client_id=1288579466242560000).

## Commands

Here are the available commands for now:

- `/new-game`: Creates a new game and writes all player records to DynamoDB. Example record:

    ```json
    {
      "UserId": {
        "S": "623440987023810570"
      },
      "StateComponent": {
        "S": "sXDwyrS6M8O#Game#Player#Inventory#Item#Armor#Equipped#Shield"
      },
      "State": {
        "NULL": true
      }
    }
    ```

## Progress

### Completed Tasks:
- [x] Created Discord bot Lambda
- [x] Created command manager Lambda
- [x] Developed the Dungeons and Llamas API (to handle interactions with game logic)
- [x] Set up DynamoDB data modeling for player inventory
- [x] Integrated bot → API → database

### Upcoming Tasks:
- [ ] Build out further data models for GAME, ENEMY, ROUNDS, LEVELS, NPC
- [ ] Add additional records for GAME, ENEMY, ROUNDS, LEVELS, NPC components to DynamoDB on `/new-game`
- [ ] Integrate with Bedrock
- [ ] Begin releasing to production
- [ ] Turn serverless Discord bot infrastructure into a Terraform module

## Developer Docs

### Initial Project Setup Commands
To set up the project environment:
```
doppler setup -p {project_name} -c {config_name}
```

### Useful Commands:
- `just apply <module>` : Apply "module"
- `just build-lambdas` : Build Lambdas
- `just deploy-lambdas` : Deploy Lambdas

### Links

- [LLM Sampling Docs](https://artefact2.github.io/llm-sampling/index.xhtml)
