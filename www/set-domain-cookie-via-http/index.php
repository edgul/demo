<?php
setcookie("somepath", "val", [
    'expires' => time() + (86400 * 30), // 86400 = 1 day
    'path' => '/some/path',
    'secure' => true,
    'httponly' => true,
    'samesite' => 'None',
    'domain' => 'localhost'
]);

?>

<!-- Serve me with `php -S localhost:8000` -->
<!DOCTYPE html>
Mostly empty content, this page sets cookies via http
