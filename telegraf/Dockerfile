FROM telegraf:1.30

RUN apt-get update && apt-get install -y python3 && rm -rf /var/lib/apt/lists/*

COPY telegraf.conf /etc/telegraf/telegraf.conf
COPY fake_fan.py /scripts/fake_fan.py
RUN chmod +x /scripts/fake_fan.py

CMD ["telegraf", "--config", "/etc/telegraf/telegraf.conf"]