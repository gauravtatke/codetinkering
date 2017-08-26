var head1 = document.querySelector('#hover');
var head2 = document.querySelector('#click');
var head3 = document.querySelector('#doubleclick');

//console.log("Connected!");

head1.addEventListener('mouseover', function(){
    head1.textContent = "Mouse currently over";
    head1.style.color = "red";
})

head1.addEventListener('mouseout', function(){
    head1.textContent = "HOVER OVER ME";
    head1.style.color = "black";
})

head2.addEventListener("click", function(){
    head2.textContent = "Click on";
    head2.style.color = "blue"
})
