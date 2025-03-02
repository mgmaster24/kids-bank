# kids-bank-sam

This project contains source code and supporting files for a serverless application that you can deploy with the SAM CLI. It includes the following files and folders:

- `kids-bank-app/Cargo.toml` - Project configuration file.
- `kids-bank-app/src/bin/` - The directory containing the different endpoints for Apigateway
- `kids-bank-app/src/lib.rs` - A common library that contains functionality used by multiple endpoints
- `./cfn/` - A directory that contains the YAML files for AWS resource definitions and scripts for building and deploying the application.

The application uses several AWS resources, including DynamoDB, Lambda functions and API Gateway API. These resources are defined in: 
- `./cfn/kidsbank_api.yaml` 
- `./cfn/account_ddb.yaml` 
- `./cfn/authorizer.yaml` 

## Requirements
* This template was tested with Rust v1.64.0 and above.

## Prerequisites
The following tools and binaries are required to build and deploy this project.
* SAM CLI - [Install the SAM CLI](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-sam-cli-install.html)
* Docker - [Install Docker community edition](https://hub.docker.com/search/?type=edition&offering=community)
* [Rust](https://www.rust-lang.org/) version 1.64.0 or newer
* [cargo-lambda](https://github.com/cargo-lambda/cargo-lambda) for cross-compilation

## Deploying Kids Bank
To simplify the build and deployment of this application three scripts were created. 
**The template definitions assume you will run these scripts from the `./cfn/` directory.**
- `./cfn/build.sh`
- `./cfn/validate.sh`
- `./cfn/deploy.sh`

### build.sh
This script runs `sam build` for the custom authorizer and the Apigateway application code.

### validate.sh
This script runs `sam validate` for all the yaml resource files.

### deploy.sh
This scripts deploys the application to the account your terminal is authenticated for.

### Use the SAM CLI to build and test locally
I haven't tested local deployments yet.  I will test this in the future and update this section

**TODO**
- [] Review the below section and provid steps for local testing

Build your application with the `sam build` command.

```bash
kids-bank-sam$ sam build
```

The SAM CLI builds the Rust app based on `kids-bank-app/Cargo.toml`, creates a deployment package, and saves it in the `.aws-sam/build` folder.

Test a single function by invoking it directly with a test event. An event is a JSON document that represents the input that the function receives from the event source. Test events are included in the `events` folder in this project.

Run functions locally and invoke them with the `sam local invoke` command.

```bash
kids-bank-sam$ sam local invoke HelloWorldFunction --event events/event.json
```

The SAM CLI can also emulate your application's API. Use the `sam local start-api` to run the API locally on port 3000.

```bash
kids-bank-sam$ sam local start-api
kids-bank-sam$ curl http://localhost:3000/
```


## Resources

See the [AWS SAM developer guide](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/what-is-sam.html) for an introduction to SAM specification, the SAM CLI, and serverless application concepts.

