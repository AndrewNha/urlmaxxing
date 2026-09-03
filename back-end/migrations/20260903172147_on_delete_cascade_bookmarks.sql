-- Add migration script here
ALTER TABLE bookmarks
DROP CONSTRAINT bookmarks_user_id_fkey;

ALTER TABLE bookmarks
ADD CONSTRAINT bookmarks_user_id_fkey
FOREIGN KEY (user_id)
REFERENCES users(id)
ON DELETE CASCADE;

ALTER TABLE bookmarks
ALTER COLUMN user_id SET NOT NULL;
