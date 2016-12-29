INSERT INTO entries (title, post, word_count)
VALUES
  ('A Tale of Three Words', 'One, Two, Three', 3),
  ('The Last Stand', 'There is no last stand everyone just lies', 8),
  ('No Words', NULL, 0);


INSERT INTO tags (name)
VALUES
  ('algorithms'),
  ('argriculture'),
  ('bahamas'),
  ('crud'),
  ('command-line'),
  ('database'),
  ('sqlite'),
  ('sqlite3'),
  ('json'),
  ('messagepack'),
  ('drivel'),
  ('vim'),
  ('void');


INSERT INTO entry_tags (id, name)
VALUES
  (1, 'drivel'),
  (2, 'drivel'),
  (3, 'void');
