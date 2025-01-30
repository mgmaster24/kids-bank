sam build \
  --beta-features \
  --no-cached \
  --template-file authorizer.yaml \
  --region "us-east-1" \
  --build-dir out/authorizer
  
sam build \
  --beta-features \
  --no-cached \
  --template-file kidsbank_api.yaml \
  --region "us-east-1" \
  --build-dir out/api
