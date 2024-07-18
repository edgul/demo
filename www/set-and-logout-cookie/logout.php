<?php
setcookie("name", "", [
    'expires' => "Thu, 01 Jan 1970 00:00:01 GMT", // begining of time, expired 
    'path' => '/',
    'secure' => true,  // Change to true if you want the cookie to be sent only over secure HTTPS connections
    'httponly' => true, // HttpOnly attribute
    'samesite' => 'Lax' // Optional, for additional security
]);
http_response_code(302);
?>

<!-- Serve me with `php -S localhost:8000` -->
<!DOCTYPE html>
This page should wipe a cookie with the name "name"
