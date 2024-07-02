#include <utility>
#include <vector>

unsigned int number(const std::vector<std::pair<int, int>>& busStops){
  int ppl = 0;
  for (int i = 0; i < busStops.size(); i++)
  {
    ppl += busStops[i].first - busStops[i].second;
  }
  return ppl;
}