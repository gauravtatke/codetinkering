// restart game button
var restart = document.querySelector('#bt')

// grab all squares
var squares = document.querySelectorAll('td')

// clear all the squares
function clearBoard(){
    for (var i = 0; i < squares.length; i++) {
        squares[i].textContent = ''
    }
}

restart.addEventListener('click', clearBoard)
// check the square marker

// Below is one method of changing marker. Below is just one cell.
// var cellOne = document.querySelector('#one');
// cellOne.addEventListener('click', function(){
//     if (cellOne.textContent === '') {
//         cellOne.textContent = 'X';
//     }
//     else if (cellOne.textContent === 'X') {
//         cellOne.textContent = 'O';
//     }
//     else{
//         cellOne.textContent = '';
//     }
// })

function changeMarker(){
    if (this.textContent === '') {
        this.textContent = 'X';
    }else if (this.textContent === 'X') {
        this.textContent = 'O';
    }else {
        this.textContent = ''
    }
}

// for loop to add event listener to all cells
for (var i = 0; i < squares.length; i++) {
    squares[i].addEventListener('click', changeMarker);
}
