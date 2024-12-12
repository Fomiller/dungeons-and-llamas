-- Your SQL goes here
ALTER TABLE "embeddings" DROP COLUMN "embedding";
ALTER TABLE "embeddings" ADD COLUMN "user_id" TEXT NOT NULL;
ALTER TABLE "embeddings" ADD COLUMN "game_id" TEXT NOT NULL;
ALTER TABLE "embeddings" ADD COLUMN "text" TEXT NOT NULL;
ALTER TABLE "embeddings" ADD COLUMN "embedding" VECTOR NOT NULL;

