<?php
setcookie("somepath", "val", [
    'expires' => time() + (86400 * 30), // 86400 = 1 day
    'path' => '/some/path',
    'secure' => true,  // Change to true if you want the cookie to be sent only over secure HTTPS connections
    'httponly' => true, // HttpOnly attribute
    'samesite' => 'None' // Optional, for additional security
]);

setcookie("nopath", "val", [
    'expires' => time() + (86400 * 30), // 86400 = 1 day
    'path' => '/',
    'secure' => true,  // Change to true if you want the cookie to be sent only over secure HTTPS connections
    'httponly' => true, // HttpOnly attribute
    'samesite' => 'Lax' // Optional, for additional security
]);
?>

<!-- Serve me with `php -S localhost:8000` -->
<!DOCTYPE html>
Mostly empty content, this page sets cookies via http
