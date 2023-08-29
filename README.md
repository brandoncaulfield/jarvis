# JARVIS

Building the thing that builds the things.

## Description

A project to hopefully help you create your own AI powered CLI that you control from model to output.

### Run the server
```npm run dev```

**Example Request**
```
curl --request POST \
  --url http://localhost:5004/generate \
  --header 'Content-Type: application/json' \
  --data '{
	"prompt": "write a simple api in express js using typescript"
}'
```
