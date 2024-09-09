CREATE TABLE parties (
  id INTEGER PRIMARY KEY,
  name VARCHAR NOT NULL
);

CREATE TABLE framework_agreements (
  id INTEGER PRIMARY KEY,
  title VARCHAR NOT NULL,
  effective_date DATE NOT NULL
);

CREATE TABLE contracts (
  id INTEGER PRIMARY KEY,
  framework_agreement_id INTEGER NOT NULL,
  title VARCHAR NOT NULL,
  effective_date DATE NOT NULL,
  seller_id INTEGER NOT NULL,
  buyer_id INTEGER NOT NULL,
  FOREIGN KEY(framework_agreement_id) REFERENCES framework_agreements(id)
  FOREIGN KEY(seller_id) REFERENCES parties(id),
  FOREIGN KEY(buyer_id) REFERENCES parties(id)
);
