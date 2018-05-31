function createGame() {
  $.ajax({
    url: 'http://localhost:8088/game',
    success: function(game) {
      document.location.href = 'http://localhost:8088/play/' + game.id;
    },
    error: function(e, status, error) { console.log('Error Loading Game'); }
  });
}
