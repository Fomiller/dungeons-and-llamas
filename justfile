import? 'just/base.just'
import? 'just/terraform.just'

project := "dungeons-and-llamas"
infraDir := "infra/modules/aws/"
env := "dev"

fetch:
    curl https://raw.githubusercontent.com/Fomiller/justfiles/refs/heads/main/base.just > just/base.just
    curl https://raw.githubusercontent.com/Fomiller/justfiles/refs/heads/main/terraform.just > just/terraform.just

################################
######### PROJECT CMDS #########
################################

cargo-run app:
    doppler run -- \
    cargo run --manifest-path src/Cargo.toml {{app}}

cargo-add app crate:
    cargo add --manifest-path src/{{app}}/Cargo.toml {{crate}}

cargo-test args="": 
    doppler run -- \
    cargo test --manifest-path src/Cargo.toml {{args}}

cargo-new path: 
    cargo new src/{{ path }}

[no-cd]
@cargo-lambda-new lambda: 
    cargo lambda new {{ lambda }}
    
build-lambdas:
    cargo lambda build \
    --target aarch64-unknown-linux-gnu \
    --release \
    --output-format zip \
    --manifest-path  src/Cargo.toml \
    --lambda-dir infra/modules/aws/lambda/bin/

deploy-lambdas: build-lambdas
    doppler run \
    --name-transformer tf-var  \
    -- terragrunt apply \
    -auto-approve \
    --terragrunt-working-dir {{infraDir}}/lambda

update-commands:
    aws s3 cp src/data/commands.json s3://fomiller-dev-dungeons-and-llamas/data/commands.json

bacon:
    bacon --project src/
    
[no-cd]
diesel cmd:
    doppler run --command='diesel --database-url postgres://$RDS_USERNAME:$RDS_PASSWORD@localhost:5432/$DATABASE_NAME {{cmd}}'

ssh-tunnel:
    doppler run --command='ssh -i $PRIVATE_KEY_PATH -L 5432:$DATABASE_ENDPOINT:5432 ec2-user@$BASTION_HOST -N -f'

kill-ssh:
    kill $(ps aux | grep ssh | grep -v grep | awk '{print $2}')

