<?php
setcookie("name", "value", [
    //'expires' => 0, // no expiry, session cookie 
    'path' => '/',
    'secure' => true,  // Change to true if you want the cookie to be sent only over secure HTTPS connections
    'httponly' => true, // HttpOnly attribute
    'samesite' => 'Lax' // Optional, for additional security
]);

?>

<!-- Serve me with `php -S localhost:8000` -->
<!DOCTYPE html>
This page sets a cookie name=value
Navigate to logout.php to send attempt to wipe the cookie with an expired cookie.
