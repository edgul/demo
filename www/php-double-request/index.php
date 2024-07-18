<?php

// attempt 1
// header("Content-type: image/gif");
// readfile('stick.gif');

// attempt 2
$im = imagecreatefrompng("test.png");
header('Content-Type: image/png');
imagepng($im);

// attempt 3
// Set the content type header - in this case image/png
// header('Content-Type: image/png');
// // Create an image with dimensions 200x50
// $width = 200;
// $height = 50;
// $image = imagecreatetruecolor($width, $height);
// // Allocate a color for the background
// $bg_color = imagecolorallocate($image, 0, 102, 204); // Blue background
// imagefill($image, 0, 0, $bg_color);
// // Allocate a color for the text
// $text_color = imagecolorallocate($image, 255, 255, 255); // White text
// // Define the text to draw
// $text = "Hello, World!";
// // Define the font size
// $font_size = 4; // Size can be 1-5 for built-in fonts
// // Calculate the position of the text to center it
// $text_width = imagefontwidth($font_size) * strlen($text);
// $text_height = imagefontheight($font_size);
// $x = ($width - $text_width) / 2;
// $y = ($height - $text_height) / 2;
// // Draw the text
// imagestring($image, $font_size, $x, $y, $text, $text_color);
// // Output the image as a PNG
// imagepng($image);
// // Free up memory
// imagedestroy($image);
?>
