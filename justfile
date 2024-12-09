set export 

infraDir := "infra/modules/aws"

clean:
    find . -name "_.*.gen.tf" -type f | xargs -r rm -rv
    find . -name ".terraform.lock.hcl" -type f | xargs -r rm -rv
    find . -name ".terraform" -type d | xargs -r rm -rv
    find . -name ".terragrunt-cache" -type d | xargs -r rm -rv

doppler-switch env:
    doppler setup -p dungeons-and-llamas -c {{env}}

login env:
    doppler run \
    -- assume-role login -p {{env}}Terraform

login-docker env:
    doppler run \
    --preserve-env="AWS_ASSUME_CONFIG_DIR" \
    -- assume-role login -p {{env}}Terraform

output-module-groups:
    doppler run \
    --name-transformer tf-var  \
    -- terragrunt output-module-groups \
    --terragrunt-working-dir {{infraDir}}

init dir:
    doppler run \
    --name-transformer tf-var  \
    -- terragrunt init \
    --reconfigure \
    --terragrunt-working-dir {{infraDir}}/{{dir}}
    
init-all:
    doppler run \
    --name-transformer tf-var  \
    -- terragrunt run-all init \
    --terragrunt-working-dir {{infraDir}}

validate dir:
    doppler run \
    --name-transformer tf-var  \
    -- terragrunt validate \
    --terragrunt-working-dir {{infraDir}}/{{dir}}

validate-all:
    doppler run \
    --name-transformer tf-var  \
    -- terragrunt validate \
    --terragrunt-working-dir {{infraDir}}
    
plan dir:
    doppler run \
    --name-transformer tf-var  \
    -- terragrunt plan \
    --terragrunt-working-dir {{infraDir}}/{{dir}}

plan-all:
    doppler run \
    --name-transformer tf-var  \
    -- terragrunt run-all \
    plan --terragrunt-working-dir {{infraDir}}
    
apply dir:
    doppler run \
    --name-transformer tf-var  \
    -- terragrunt apply \
    -auto-approve \
    --terragrunt-working-dir {{infraDir}}/{{dir}}
    
apply-all:
    doppler run \
    --name-transformer tf-var  \
    -- terragrunt run-all apply \
    --terragrunt-working-dir {{infraDir}} \
    --terragrunt-non-interactive

destroy dir:
    doppler run \
    --name-transformer tf-var  \
    -- terragrunt destroy \
    -auto-approve \
    --terragrunt-working-dir {{infraDir}}/{{dir}}
    
destroy-all:
    doppler run \
    --name-transformer tf-var  \
    -- terragrunt run-all \
    destroy --terragrunt-working-dir {{infraDir}}

state-list dir:
    doppler run \
    --name-transformer tf-var  \
    -- terragrunt state list \
    --terragrunt-working-dir {{infraDir}}/{{dir}}
    
fmt:
    doppler run \
    --name-transformer tf-var  \
    -- terraform fmt \
    --recursive
    
@init-module dir:
    mkdir -p {{infraDir}}/{{dir}}/env-config/us-east-1
    
    touch {{infraDir}}/{{dir}}/env-config/common.tfvars
    touch {{infraDir}}/{{dir}}/env-config/us-east-1/common.tfvars
    touch {{infraDir}}/{{dir}}/env-config/us-east-1/dev.tfvars
    touch {{infraDir}}/{{dir}}/env-config/us-east-1/prod.tfvars
    touch {{infraDir}}/{{dir}}/_outputs.tf
    touch {{infraDir}}/{{dir}}/_inputs.tf
    touch {{infraDir}}/{{dir}}/_locals.tf
    touch {{infraDir}}/{{dir}}/_data.tf
    touch {{infraDir}}/{{dir}}/_variables.tf
    touch {{infraDir}}/{{dir}}/main.tf
    touch {{infraDir}}/{{dir}}/terragrunt.hcl
    
    echo 'asset_name = "{{dir}}"' >> {{infraDir}}/{{dir}}/env-config/common.tfvars
    echo 'locals {}' >> {{infraDir}}/{{dir}}/_locals.tf
    echo 'environment = "dev"' >> {{infraDir}}/{{dir}}/env-config/us-east-1/dev.tfvars
    echo 'environment = "prod"' >> {{infraDir}}/{{dir}}/env-config/us-east-1/prod.tfvars
    echo 'include "root" { path = find_in_parent_folders() }' >> {{infraDir}}/{{dir}}/terragrunt.hcl
    echo 'data "aws_caller_identity" "current" {}' >> {{infraDir}}/{{dir}}/_data.tf
    echo 'data "aws_region" "current" {}' >> {{infraDir}}/{{dir}}/_data.tf
    @# {{infraDir}}/{{dir}} created.

######### project cmds #########
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
    bacon --path src
    
[no-cd]
diesel cmd:
    doppler run --command='diesel --database-url postgres://$RDS_USERNAME:$RDS_PASSWORD@localhost:5432/$DATABASE_NAME {{cmd}}'

ssh-tunnel:
    doppler run --command='ssh -i $PRIVATE_KEY_PATH -L 5432:$DATABASE_ENDPOINT:5432 ec2-user@$BASTION_HOST -N -f'

kill-ssh:
    kill $(ps aux | grep ssh | grep -v grep | awk '{print $2}')

