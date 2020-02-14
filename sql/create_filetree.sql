/* WARNING -- this is destructive */

/* Add the root value to Google Drive */
/* NOTE -- this will not work for shared drives yet */
DROP TABLE if exists filetree;
CREATE TABLE filetree (
    file_id varchar(255),
    path ltree
);
create index if not exists tree_path_idx on filetree using gist (path);

/* Add all root/missing "files". */

insert into files
select t1.parent_id as id, 'root' as name from (select distinct parent_id from parents) as t1 left join files on t1.parent_id = files.id where id is null and parent_id like '0A%Uk9PVA';

insert into files
select t1.parent_id as id, 'missing' as name from (select distinct parent_id from parents) as t1 left join files on t1.parent_id = files.id where id is null and parent_id not like '0A%Uk9PVA';

WITH RECURSIVE 
get_parents(id, file_id) as (
  /* get all files with no parents */
  Select cast (db_id as TEXT), files.id from files left join parents on files.id = parents.file_id where parent_id is null
  UNION ALL
  SELECT concat(get_parents.id, '.', files.db_id), files.id
    FROM parents JOIN get_parents ON parents.parent_id=get_parents.file_id JOIN files ON parents.file_id = files.id
  ) 

insert into filetree
select file_id, cast (id as ltree) from get_parents; /* breadth first */

/* generates path names for everything and puts it into a separate table
Would put this in files if I was better at sql */
/* todo -- use a separator other than > for disambiguation maybe */
drop table if exists path_names;
SELECT id, label into path_names from(
  SELECT string_agg(f.name, ' > ' ORDER BY t.ord) as label, e.path as path
    FROM filetree e
    JOIN regexp_split_to_table(e.path::text, '[.]') WITH ORDINALITY t(item, ord) ON true
    JOIN files f ON f.db_id = t.item::int
    GROUP by e.path
    ORDER BY label) as table1 /* remove order by? */
  join files on subpath(path, -1)::text::integer = files.db_id ;

create index if not exists path_id on path_names(id)
