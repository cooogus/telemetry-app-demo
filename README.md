Telemetry App Demo – OpenConfig (Telegraf) + OpenTelemetry Pipeline

This project is a proof-of-concept telemetry pipeline demonstrating how to collect fake hardware metrics (e.g., fan speeds from a simulated Junos/OpenConfig device) using Telegraf, route them through the OpenTelemetry Collector, store/query them in Prometheus, and visualize them in Grafana.

It is designed to test an OpenConfig → Telegraf → OTEL → Prometheus → Grafana workflow with easily swappable fake or real data sources.

Requirements

- Docker
- Docker Compose
- (Optional) Python 3 if running fake_fan.py locally for testing

Getting Started
1. Clone the repo
git clone git@github.com:cooogus/telemetry-app-demo.git

2. Build and start services
docker compose up -d --build 
