#!/bin/bash

set -xe

# Setup cluster
kind delete cluster || true
kind create cluster --config=./kind-config.yaml

# Install a postgres operator
git clone git@github.com:tembo-io/tembo.git
cd tembo
git checkout 51dd3b6ea85d034223a6079b6df5519377c94f60
helm install coredb-operator ./coredb-operator/charts/coredb-operator
cd ..
rm -rf tembo

# Deploy traefik v3
helm repo add traefik https://traefik.github.io/charts
helm repo update
kubectl create namespace traefik || true
helm upgrade --install --namespace=traefik --values=./traefik-values.yaml traefik traefik/traefik

# Deploy postgres, waiting for operator to be ready
until kubectl apply -f ./postgres.yaml; do
  echo "waiting for postgres to apply..."
  sleep 1
done

# Deploy ingress
kubectl apply -f ./ingress.yaml

# wait for password to be ready
until kubectl get secrets -n customer-1 sample-coredb-connection; do
  echo "waiting for postgres password to be ready..."
  sleep 1
done

export PASSWORD=$(kubectl get secrets -n customer-1 sample-coredb-connection -o json | jq -r '.data.password' | base64 --decode)

set +x
echo "==============="
echo ""
echo "Check for pod to be ready:"
echo ""
echo "kubectl get pods -n customer-1"
echo ""
echo "Reproduce by deleting the pod:"
echo ""
echo "kubectl delete pods -n customer-1 sample-coredb-0"
echo ""
echo "while the pod is not running, try connecting:"
echo ""
echo "psql 'postgresql://postgres:${PASSWORD}@localhost:5432/postgres'"
