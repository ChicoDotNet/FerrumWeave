using System;
using Newtonsoft.Json;

var score = FerrumWeave.RustApi.Answer();
Console.WriteLine($"risk-score={score}");
Console.WriteLine($"nuget-risk-score={JsonConvert.SerializeObject(new { score })}");
