SELECT files.id, permissions.perm_type, domain, web_view_link, created_time, modified_time, owners -> 0 -> 'emailAddress' as owner, label 
from files 
inner join permissions 
on files.id = permissions.file_id 
join path_names 
on files.id = path_names.id 
where (perm_type='anyone' or perm_type = 'domain') 
and (LOWER(name) like '%secret%' 
  or LOWER(name) like '%pass%' 
  or LOWER(name) like '%credential%' 
  or LOWER(name) like '%creds%' 
  or LOWER(name) like '%login%' 
  or LOWER(name) like '%key%')
