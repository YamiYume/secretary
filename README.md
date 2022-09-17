# Secretary
Simple cryptographic app written in Rust
## Installation
Use rust toolchain of your preference, clone the repository and do Cargo Build --release, launch.
## Caesar Encryptor
Simple Displacement encriptor, with a slider for key selection, it receive lowercase text entry.
### Use
Introduce a plaintext exclusively in lowercase and spaces, choose a key in the slider, the tool will automatically update your ciphertext with every change, you can copy the key or the ciphertext by clicking the buttons, the output is a single uppercase word.
### Equation
![equation](https://latex.codecogs.com/svg.image?f_k(x)&space;=&space;(x&space;&plus;&space;k)&space;%&space;26)
### False positive
In particular this cipher limited with the ky ranges from 1 to 25 we used have no false positive
## Caesar Decryptor
Decryptor for Caesar encryptor
### Use
Introduce a ciphertext exclusively in uppercase and a single word, choose a key in the slider, the tool will automatically update your plaintext with every change, you can copy the key or the plaintext by clicking the buttons, the output is a single lowercase word.
### Equation
![equation](https://latex.codecogs.com/svg.image?f_k(x)&space;=&space;(x&space;-&space;k)&space;%&space;26)
## Caesar Attack
A brute force attack for Caesar encryptions
### Use
Introduce a ciphertext exclusively in uppercase and a single word, the table will be generated with every change, simply search the answer and can copy the answer and key by clicking the side buttons
### Equation
![equation](https://latex.codecogs.com/svg.image?A&space;=&space;\{(k,&space;f_k(x)):&space;0&space;<&space;k&space;<&space;26\})
## Afin Encryptor
Displacement and multiplicative encriptor, two sliders for key selection.
### Use
Introduce plaintext in lowercase and spaces, select the keys with the sliders, the sliders will show if the key is valid, it update automatically with the changes,
use the buttons for copy key and ciphertext, the output is single word uppercase.
### Equation
![equation](https://latex.codecogs.com/svg.image?f_{k_1k_2}(x)=&space;(x&space;*&space;k_1&space;&plus;&space;k_2)&space;%&space;26)
### False Positive
There is certain possibility of false positive but it is very unlikely given the variety of letters in common words
## Afin Decryptor
Decryptor for Afin Encryptor
### Use
Insert ciphertext as single word uppercase, select your key with the sliders, it will update automatically, the Ourput will be single word in lowercase
### Equation
![equation](https://latex.codecogs.com/svg.image?f_{k_1k_2}(x)=&space;((x&space;-&space;k_2)&space;*&space;k_1^{-1})&space;%&space;26)
## Afin Attack 
a brute force attack fot the afin encryptor
### Use
Introduce a ciphertext exclusively in uppercase and a single word, the table will be generated with every change, simply search the answer and can copy the answer and key by clicking the side buttons
### Equation
![equation](https://latex.codecogs.com/svg.image?(&space;f_(k_1k_2)(x),&space;k_1,&space;k_2&space;):&space;gcd(k1,&space;26)&space;=&space;1)
## Vigenere Encryptor
Displacement encryptor with multiple displacements
### Use
Introduce plaintext in lowercase and spaces, introduce a key as lowercase single word, it update automatically as you write, use the buttons for copy ciphertext or key
### Concept
The Vigenere encrption applies the same concept as the Caesar Encryptor but it takes a different character through the encryption of blocks of the same size of the key, iterating over the key ccharavters and using them as key
### False Positive
certains keys containing a and z wont cipher the data they are applied to, that can lead to poorly encripted data.
## Vigenere Decryptor
Decryptor for Vigenere Encrptor
### Use
Introduce ciphertext as a single uppercase word, introduce key as asingle lowercase word, it updates automatically, use the buttons for copy the plaintext or the key.
### Concept
it applies the same Concept as the Caesar Decryptor in blocks of text of lenght equal to the key, iterating through it and using the characters as the key for the equation.
## Permutation Encryptor
Simple permutation encryptor
### Use
Introduce plaintext in lowercase and spaces, introduce the key as single word digits, the key must contain all the digits smaller than his max and no include the 0, the maximum lenght for the key is 9, it updates automatically if have a valid key, use the buttons for copy the ciphertext or the key
### Concept
The permutation encryptor simply divides the plaintext by block and apply the key as a permutation to that block, it fills with "a" before apply the permutation in case some lenght is missing.
### False Positive
Ordered keys will produce totally ordered or semi ordereds blocks, also some words as anagrams can still be reveal withother keys, even small 2 ordered digits can lead to reveal a lot of important parts in long texts
## Permutation Decryptor
A Decryptor for the Permutation Encryptor
### Use
Introduce ciphertext as uppercase blocks divided by a single space, introduce the key as single word digits, the key must contain all the digits smaller than his max and no include the 0, the maximum lenght for the key is 9, it updates automatically if have a valid key, use the buttons for copy the plaintext or the key
### Concept
It simply arrange back the chars of each block according to the key, simply inserting them back in position.
## Hill Encryptor
a matrix multiplication based encryptor for grayscaled images
### Use
Introduce a absolute image path as input (only tested on linux) and use the button for loas it, introduce the key that must be 16 or 9 space separated numbers in range of 0 to 255, it will automatically generate the result, you can use the button save fro save it, it will save in the smae location then the source image but with cipher ass prefix for his name
### Concept
The hill encryptor turn the key on a matrix of dimensions 3 x 3 and 4 x 4, it multiply the pixels values of the image arranged in a matrix of same size with the key and replace them with the result modulo 256
### False Positive
I had no identify any false positive in this cipher yet.
