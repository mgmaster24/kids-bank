REGION="us-east-1"

if [ -z $1 ]; then
  echo "Token ARN must be provided"
  exit 1
fi

# Deploy the custom authorizer
sam deploy \
  --template-file out/authorizer/template.yaml \
  --stack-name kids-bank-authorizer-stack \
  --region $REGION \
  --s3-bucket kids-bank-auth \
  --capabilities CAPABILITY_IAM \
  --parameter-overrides \
  ParameterKey=TokenSecretArn,ParameterValue=$TOKEN_SECRET_ARN

# Deploy the accounts table
sam deploy \
  --template-file account_ddb.yaml \
  --stack-name kids-bank-accounts-table-stack \
  --region $REGION

ACCOUNTS_TABLE_STACK_NAME="kids-bank-accounts-table-stack"
CUSTOM_AUTHORIZER_STACK_NAME="kids-bank-authorizer-stack"
QUERY='Stacks[0].Outputs[*].{key:OutputKey,value:OutputValue}'

# Get the Accounts Table Name from the stack
aws cloudformation describe-stacks \
  --stack-name $ACCOUNTS_TABLE_STACK_NAME \
  --region $REGION --no-paginate --query $QUERY \
  --output json | jq 'from_entries?' > stack_output.json && cat stack_output.json
ACCOUNT_TABLE_NAME=$(cat stack_output.json | jq -r '.AccountsTable')

# Get the authorizer function ARN from the stack
aws cloudformation describe-stacks \
  --stack-name $CUSTOM_AUTHORIZER_STACK_NAME \
  --region $REGION --no-paginate --query $QUERY \
  --output json | jq 'from_entries?' > stack_output.json && cat stack_output.json
CUSTOM_AUTHORIZER_FUNCTION_ARN=$(cat stack_output.json | jq -r '.CustomAuthorizerFunctionArn')

CUSTOM_AUTH_PK="CustomAuthorizerFunctionArn"
ACCOUNTS_TABLE_PK="AccountsTableName"

# Deploy the Kids Bank API
echo "TOKEN_ARN: $1"
sam deploy \
  --template-file out/api/template.yaml \
  --stack-name kids-bank-api \
  --parameter-overrides \
  ParameterKey=$CUSTOM_AUTH_PK,ParameterValue=$CUSTOM_AUTHORIZER_FUNCTION_ARN \
  ParameterKey=$ACCOUNTS_TABLE_PK,ParameterValue=$ACCOUNT_TABLE_NAME \
  ParameterKey=TokenSecretArn,ParameterValue=$1 \
  --region $REGION \
  --s3-bucket kids-bank-api \
  --capabilities CAPABILITY_IAM

rm -rf stack_output.json
