var number = function(busStops){
    var ppl = 0;
    for (var stop in busStops)
      {
        ppl += stop[0] - stop[1];
      }
    return ppl;
  }