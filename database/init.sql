CREATE TABLE IF NOT EXISTS assets (
  address TEXT NOT NULL,
  name TEXT NOT NULL,
  symbol TEXT NOT NULL,
  decimals INTEGER NOT NULL,
  logo_url TEXT NOT NULL,
  price FLOAT NOT NULL,
  PRIMARY KEY (address)
);

CREATE TABLE IF NOT EXISTS plugins (
  address TEXT NOT NULL,
  symbol TEXT NOT NULL,
  total_supply FLOAT NOT NULL,
  reserve0 FLOAT NOT NULL,
  reserve1 FLOAT NOT NULL,
  gauge_address TEXT NOT NULL,
  tvl FLOAT NOT NULL,
  token0_address TEXT NOT NULL,
  token0 json NOT NULL,
  token1_address TEXT NOT NULL,
  token1 json NOT NULL,
  PRIMARY KEY (address),
  FOREIGN KEY (token0_address) REFERENCES assets (address),
  FOREIGN KEY (token1_address) REFERENCES assets (address)
);

CREATE TABLE IF NOT EXISTS aprs (
  plugin_address TEXT NOT NULL,
  token_address TEXT NOT NULL,
  apr FLOAT NULL,
  min_apr FLOAT NULL,
  max_apr FLOAT NULL,
  symbol TEXT NOT NULL,
  logo_url TEXT NOT NULL,
  PRIMARY KEY (plugin_address, token_address),
  FOREIGN KEY (plugin_address) REFERENCES plugins (address),
  FOREIGN KEY (token_address) REFERENCES assets (address)
);

CREATE TABLE IF NOT EXISTS gauges (
  address TEXT NOT NULL,
  decimals INTEGER NOT NULL,
  total_supply FLOAT NOT NULL,
  bribe_address TEXT NOT NULL,
  reward FLOAT NOT NULL,
  median_tbv FLOAT NOT NULL,
  min_tbv FLOAT NOT NULL,
  max_tbv FLOAT NOT NULL,
  votes FLOAT NOT NULL,
  min_apr FLOAT NOT NULL,
  max_apr FLOAT NOT NULL,
  plugin_address TEXT NOT NULL,
  PRIMARY KEY (address),
  FOREIGN KEY (plugin_address) REFERENCES plugins (address)
);

CREATE TABLE IF NOT EXISTS bribes (
  bribe_address TEXT NOT NULL,
  reward_amount FLOAT NOT NULL,
  token_address TEXT NOT NULL,
  token json NOT NULL,
  plugin_address TEXT NOT NULL,
  PRIMARY KEY (bribe_address, plugin_address, token_address),
  FOREIGN KEY (plugin_address) REFERENCES plugins (address),
  FOREIGN KEY (token_address) REFERENCES assets (address)
);