import pandas as pd
import plotly.express as px
import json
import os

# Load all hyperfine results
results = []
for file in os.listdir("hyperfine_results"):
    if file.endswith(".json"):
        with open(f"hyperfine_results/{file}") as f:
            data = json.load(f)
            binary = file.split("_")[0]
            input_file = "_".join(file.split("_")[1:]).replace(".json", ".txt")
            results.append(
                {
                    "file": input_file,
                    "binary": binary,
                    "mean_time": data["results"][0]["mean"],
                    "stddev": data["results"][0]["stddev"],
                }
            )

df = pd.DataFrame(results)

# Create interactive plot
fig = px.bar(
    df,
    x="binary",
    y="mean_time",
    error_y="stddev",
    color="file",
    barmode="group",
    title="Branch Prediction Benchmark Results",
    labels={"mean_time": "Time (seconds)", "binary": "Binary Version"},
    hover_data=["stddev"],
    color_discrete_map={
        "90-10_likely.txt": "#636EFA",
        "50-50.txt": "#EF553B",
        "10-90_unlikely.txt": "#00CC96",
    },
)

fig.update_layout(
    hovermode="x unified",
    yaxis_title="Execution Time (seconds)",
    xaxis_title="",
    legend_title="Input File",
)

fig.show()
fig.write_html("benchmark_results.html")
