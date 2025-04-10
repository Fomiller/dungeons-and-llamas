<p align="center">
<img src="src/_data/DNL_README.jpg" alt="Dungeons and Llamas" width="600"/>
</p>


<!-- [![crates.io](https://img.shields.io/crates/v/my_crate.svg)](https://crates.io/crates/my_crate) -->
<!-- [![docs.rs](https://docs.rs/my_crate/badge.svg)](https://docs.rs/my_crate) -->
<!-- [![Build Status](https://github.com/Fomiller/dungesons-and-llamas/actions/workflows/deploy.yml/badge.svg)](https://github.com/Fomiller/dungeons-and-llamas/actions) -->
<!-- [![codecov](https://codecov.io/gh/myusername/myrepo/branch/main/graph/badge.svg)](https://codecov.io/gh/myusername/myrepo) -->
![Rust Version](https://img.shields.io/badge/rustc-1.82+-orange.svg)
<!-- [![License](https://img.shields.io/crates/l/my_crate.svg)](https://crates.io/crates/my_crate) -->


# Dungeons and Llamas
## Description
A serverless Discord bot written in Rust, leveraging AWS services such as API-Gateway, DynamoDB, Lambda, Bedrock and more! This bot creates dynamic text-based adventures, powered by LLMs, allowing users to experience unique, AI-generated stories each time they play.

Dungeons and Llamas revolves around leveraging discord's API as a UI. The player will have a variety of commands available to them while playing. 

To start a new game the user will use the `/new-game` command where they will fill in some base values about their character.  
<img src="https://github.com/Fomiller/dungeons-and-llamas/blob/develop/src/_data/new_game_cmd.png" width="50%" />  

The user can fill in the theme field with whatever they want and the AI will create content related to the theme. 

If the user ever wants to adjust the settings of their campaign they can make changes using the `/settings` command.  
<img src="https://github.com/Fomiller/dungeons-and-llamas/blob/develop/src/_data/settings_cmd.png" width="50%" />

Once a new game is started the user will be presented with a start to their campaign and the choice of 3 different weapons to begin their adventure with  
<img src="https://github.com/Fomiller/dungeons-and-llamas/blob/develop/src/_data/new_game_gen.png" width="50%" />

Currently, there are 3 different scenarios that can be generated:
Battle  
<img src="https://github.com/Fomiller/dungeons-and-llamas/blob/develop/src/_data/battle_scenario_gen.png" width="50%" />

Shop  
<img src="https://github.com/Fomiller/dungeons-and-llamas/blob/develop/src/_data/shop_scenario_gen.png" width="50%" />

Rest  
<img src="https://github.com/Fomiller/dungeons-and-llamas/blob/develop/src/_data/rest_scenario_gen.png" width="50%" />

## Installation
To add the Dungeons-and-Llamas bot to your discord follow this [link](https://discord.com/oauth2/authorize?client_id=1288579466242560000) 

## Commands
Working Commands are currently 
- `/new-game`
    - creates a new game, currently writes all records to dynamodb for player inventory,
    example:
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
- [x] create discord bot lambda
- [x] create command manager lambda
- [x] create dungeons and llamas api (api to handle all interactions with game logic)
- [x] dynamodb data modeling for player inventory
- [x] integrate bot -> api -> database
- [ ] build out further data models for GAME, ENEMY, ROUNDS, LEVELS, NPC
- [ ] add additional records for GAME, ENEMY, ROUNDS, LEVELS, NPC components to dynamodb on `/new-game`
- [ ] integrate with Bedrock
- [ ] start releasing to production
- [ ] turn serverless discord bot infrastructure into a terraform module


## Developer Docs
- inital project setup commmands
```
doppler setup -p {project_name} -c {config_name}
```
* just Commands
    * just apply "module"
    * just build-lambdas
    * just deploy-lambdas


### Links
https://artefact2.github.io/llm-sampling/index.xhtml
