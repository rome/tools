CREATE SEQUENCE IF NOT EXISTS contributions_id_seq;

CREATE TABLE "contributions" (
    "id" int4 NOT NULL DEFAULT nextval('contributions_id_seq'::regclass),
    "stripeSession" text NOT NULL,
    "email" text NOT NULL,
    "tierId" text NOT NULL,
    "tierPrice" real NOT NULL,
    "public" bool NOT NULL,
    "publicName" text NOT NULL,
    "publicComment" text NOT NULL,
    "tip" real NOT NULL,
    "twitter" text NOT NULL,
    "github" text NOT NULL,
    "discord" text NOT NULL,
    "paid" bool NOT NULL DEFAULT false,
    "createdAt" timestamp NOT NULL DEFAULT now(),
    PRIMARY KEY ("id")
);

CREATE INDEX paid_key ON "contributions" USING btree ("paid");
CREATE UNIQUE INDEX "stripeSession_key" ON "contributions" USING btree ("stripeSession");
CREATE INDEX "tierPrice_key" ON "contributions" USING btree ("tierPrice");
CREATE INDEX tip_key ON "contributions" USING btree ("tip");