# persistent-gauge
A persistent gauge service

To run this service run `docker-compose up`.

## Create Gauge
```bash
curl -X POST 0.0.0.0:80/api/gauge/create/clams
```

## Get Gauges
```bash
curl -X GET 0.0.0.0:80/api/gauge/gauges
```

## Increment Gauge
```bash
curl -X PUT 0.0.0.0:80/api/gauge/increment/clams
```

## Decrement Gauge
```bash
curl -X PUT 0.0.0.0:80/api/gauge/decrement/clams
```

## Metrics
```bash
curl 0.0.0.0:80/metrics
```

## Status
```bash 
curl 0.0.0.0:80/status
```
