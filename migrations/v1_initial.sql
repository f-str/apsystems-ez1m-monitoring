CREATE EXTENSION IF NOT EXISTS timescaledb;

CREATE TABLE IF NOT EXISTS ez1m_data {
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    p1 REAL NOT NULL, 
    e1 REAL NOT NULL, 
    t1 REAL NOT NULL,
    p2 REAL NOT NULL,
    e2 REAL NOT NULL,
    t2 REAL NOT NULL,   
    message VARCHAR(10) NOT NULL,
    device_id VARCHAR(64) NOT NULL,
};

SELECT create_hypertable('ez1m_data', by_range('time', INTERVAL '1 day'), if_not_exists => TRUE);

CREATE INDEX ix_device_time ON ez1m_data (device_id, time DESC);

ALTER TABLE ez1m_data SET (
  timescaledb.compress,
  timescaledb.compress_segmentby = 'device_id'
);

SELECT add_compression_policy('ez1m_data', INTERVAL '7 days');
