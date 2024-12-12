-- This file should undo anything in `up.sql`
ALTER TABLE "embeddings" DROP COLUMN "user_id";
ALTER TABLE "embeddings" DROP COLUMN "game_id";
ALTER TABLE "embeddings" DROP COLUMN "text";
ALTER TABLE "embeddings" DROP COLUMN "embedding";
ALTER TABLE "embeddings" ADD COLUMN "embedding" VECTOR;

