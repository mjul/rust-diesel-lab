CREATE TABLE parties (
  id INTEGER PRIMARY KEY NOT NULL,
  name VARCHAR NOT NULL
);

CREATE TABLE framework_agreements (
  id INTEGER PRIMARY KEY NOT NULL,
  title VARCHAR NOT NULL,
  effective_date DATE NOT NULL
);

CREATE TABLE contracts (
  id INTEGER PRIMARY KEY NOT NULL,
  framework_agreement_id INTEGER NOT NULL,
  title VARCHAR NOT NULL,
  effective_date DATE NOT NULL,
  seller_id INTEGER NOT NULL,
  buyer_id INTEGER NOT NULL,
  FOREIGN KEY(framework_agreement_id) REFERENCES framework_agreements(id)
  FOREIGN KEY(seller_id) REFERENCES parties(id),
  FOREIGN KEY(buyer_id) REFERENCES parties(id)
);
