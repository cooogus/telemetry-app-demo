# Telemetry App Demo – Recogni App-Sim + OpenConfig (Telegraf) → OpenTelemetry Pipeline

This project is a proof-of-concept telemetry pipeline demonstrating how to collect fake hardware and application metrics and observe them end-to-end:

**app-sim**: a Rust service simulating Recogni app metrics (chip temperature, inference latency).

**telegraf**: scrapes simulated OpenConfig/Junos metrics (e.g., fan speed) via gNMI.

**otel-collector**: receives both OTLP metrics (from app-sim) and scraped metrics (from Telegraf), processes them, and exports to Prometheus.

**prometheus**: stores time series.

**grafana**: visualizes dashboards showing how chip temperature, fan speed, and latency relate.

The design makes it easy to swap fake sources (random jitter) with real telemetry when available.

Requirements

- Docker
- Docker Compose
- (Optional) Python 3 if running fake_fan.py locally for testing

Getting Started
1. Clone the repo
git clone git@github.com:cooogus/telemetry-app-demo.git

2. Build and start services
docker compose up -d --build 
