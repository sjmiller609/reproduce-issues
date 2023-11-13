# UPDATE: resolved

https://github.com/traefik/traefik/pull/10089#issuecomment-1773515235

# Reproduce issue with psql hanging during pod restart

## Dependencies:

- Docker
- Kubernetes in Docker (kind)
- jq
- git
- helm 3
- base64

## Reproduce

- Run the setup script:
```
/bin/bash ./setup.sh
```
- Wait for pod to be ready, checking with:
```
watch kubectl get pods -n customer-1
```
- Try connecting to postgres, waiting for pod to be ready
- Delete the pod, and try connecting again (see script output)

## Debug information

- http://localhost:9000/api/rawdata
- http://localhost:9000/dashboard#/
