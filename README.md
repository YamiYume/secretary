# Secretary
Simple cryptographic app written in Rust
##Installation
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
## Afin Decryptos
Decryptor for Afin Encryptor
### Use
Insert ciphertext as single word uppercase, select your key with the sliders, it will update automatically, the Ourput will be single word in lowercase
### Equation
![equation](https://latex.codecogs.com/svg.image?f_{k_1k_2}(x)=&space;((x&space;-&space;k_2)&space;*&space;k_1^{-1})&space;%&space;26)
