-- This file should undo anything in `up.sql`
ALTER TABLE "embeddings" RENAME COLUMN "vector" TO "embedding";

