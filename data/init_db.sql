CREATE DATABASE IF NOT EXISTS `gauge@gauge` CHARACTER SET = 'UTF8mb4' COLLATE = 'utf8mb4_bin';

CREATE USER 'myuser' IDENTIFIED BY 'mypassword';
GRANT USAGE ON *.* TO 'myuser'@'%' IDENTIFIED BY 'mypassword';
GRANT ALL privileges ON `gauge@gauge`.* TO 'myuser'@'%';

FLUSH PRIVILEGES;
