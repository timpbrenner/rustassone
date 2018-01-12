function createGame() {
  $.ajax({
    url: 'http://localhost:8000/game',
    success: function(game) {
      console.log(game);
      document.location.href = 'http://localhost:8000/game/' + game.id;
    },
  });
}
