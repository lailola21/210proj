import pandas as pd
import matplotlib.pyplot as plt

# File paths
input_csv = "genre_popularity_over_time.csv"  # Path to the CSV file created by Rust code
output_image = "genre_popularity_trends.png"  # Path to save the visualization

def load_genre_trends(file_path):
    """Load the genre popularity trends from a CSV file."""
    return pd.read_csv(file_path)

def plot_genre_trends(data, output_path):
    """Plot and save the genre popularity trends."""
    plt.figure(figsize=(12, 8))

    # Group by year and genre
    pivot_data = data.pivot(index="Year", columns="Genre", values="Count").fillna(0)

    # Plot each genre
    pivot_data.plot(kind="line", ax=plt.gca())

    # Customize the plot
    plt.title("Genre Popularity Over Time", fontsize=16)
    plt.xlabel("Year", fontsize=14)
    plt.ylabel("Number of Movies", fontsize=14)
    plt.legend(title="Genres", loc="upper left", bbox_to_anchor=(1.05, 1), fontsize=10)
    plt.grid(visible=True, linestyle="--", linewidth=0.5)
    plt.tight_layout()

    # Save the plot
    plt.savefig(output_path)
    print(f"Plot saved to {output_path}")

def main():
    # Load the genre trends
    data = load_genre_trends(input_csv)

    # Plot and save the visualization
    plot_genre_trends(data, output_image)

if __name__ == "__main__":
    main()
