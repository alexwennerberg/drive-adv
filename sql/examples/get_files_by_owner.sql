select email_address, count(*) 
FROM permissions 
WHERE role = 'owner' 
GROUP BY email_address;   
