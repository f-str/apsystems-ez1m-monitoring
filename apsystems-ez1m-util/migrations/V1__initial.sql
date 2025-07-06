CREATE EXTENSION IF NOT EXISTS timescaledb;

-- Table for ez1m data with hypertable and compression
CREATE TABLE IF NOT EXISTS data_ez1m (
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    device_id VARCHAR(64) NOT NULL,
    value_key VARCHAR(16) NOT NULL,
    double_value DOUBLE PRECISION NULL DEFAULT NULL
);

SELECT create_hypertable('data_ez1m', by_range('time', INTERVAL '1 day'), if_not_exists => TRUE);

CREATE INDEX ix_device_time ON data_ez1m (device_id, time DESC);

ALTER TABLE data_ez1m SET (
  timescaledb.compress,
  timescaledb.compress_segmentby = 'device_id'
);

SELECT add_compression_policy('data_ez1m', INTERVAL '7 days');


