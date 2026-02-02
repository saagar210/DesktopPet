import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import { CoinBadge } from "../CoinBadge";

describe("CoinBadge", () => {
  it("displays the amount", () => {
    render(<CoinBadge amount={42} />);
    expect(screen.getByText("42")).toBeInTheDocument();
  });

  it("displays zero", () => {
    render(<CoinBadge amount={0} />);
    expect(screen.getByText("0")).toBeInTheDocument();
  });

  it("renders a coin SVG icon", () => {
    const { container } = render(<CoinBadge amount={10} />);
    const svg = container.querySelector("svg");
    expect(svg).toBeInTheDocument();
  });
});
