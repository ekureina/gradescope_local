# gradescope\_local

A tool to visualize and run gradescope locally using docker.
Visualization is able to be done without docker, but local runs internally
call a 'sh' script stored in the program memory which talks to docker.

Can view graderscope autograder output files as raw, or can run docker locally
in order to generate the output before displaying it.

NOTE: Not affiliated with the official gradescope product in any manner.
This project is to help develop custom autograders without the use of
gradescope's online resources.
