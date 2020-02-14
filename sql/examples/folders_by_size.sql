/* Get largest folders by number of children */
SELECT name, files.id, label as file_path, web_view_link, owners -> 0 -> 'emailAddress' as owner, children_count, created_time, modified_time 
from (select subpath(f1.path, -1)::text::int as db_id, count(*) as children_count from filetree f1
JOIN filetree f2
ON f1.path @> f2.path
JOIN files ON
F1.file_id = files.id
WHERE files.mime_type = 'application/vnd.google-apps.folder'
GROUP BY f1.path
ORDER BY count(*) desc) as t1 join files on t1.db_id=files.db_id
JOIN path_names on files.id = path_names.id;
