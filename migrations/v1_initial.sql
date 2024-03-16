CREATE EXTENSION IF NOT EXISTS timescaledb;

-- Table for ez1m data with hypertable and compression
CREATE TABLE IF NOT EXISTS ez1m_data {
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    p1 REAL NOT NULL, 
    e1 REAL NOT NULL, 
    te1 REAL NOT NULL,
    p2 REAL NOT NULL,
    e2 REAL NOT NULL,
    te2 REAL NOT NULL,   
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

-- Table for weather data with hypertable and compression
CREATE TABLE IF NOT EXISTS weather_data {
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    temperature REAL NOT NULL,
    humidity REAL NOT NULL,
    pressure REAL NOT NULL,
};

SELECT create_hypertable('weather_data', by_range('time', INTERVAL '1 day'), if_not_exists => TRUE);

CREATE INDEX ix_weather_time ON weather_data (time DESC);

ALTER TABLE weather_data SET (
  timescaledb.compress,
  timescaledb.compress_segmentby = 'time'
);

SELECT add_compression_policy('weather_data', INTERVAL '7 days');


-- Table for notification-subscriber data
CREATE TABLE IF NOT EXISTS notification_subscriber {
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(64) NOT NULL,
    firstname VARCHAR(64) NOT NULL,
    lastname VARCHAR(64) NOT NULL,
    notification_level VARCHAR(64) NOT NULL,
};
