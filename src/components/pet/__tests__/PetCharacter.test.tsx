import { describe, it, expect } from "vitest";
import { render } from "@testing-library/react";
import { PetCharacter } from "../PetCharacter";

describe("PetCharacter", () => {
  it("renders stage 1 (default) for stage 0", () => {
    const { container } = render(<PetCharacter stage={0} accessories={[]} />);
    const svg = container.querySelector("svg");
    expect(svg).toBeInTheDocument();
    // Stage 1 SVG is 150x150
    expect(svg?.getAttribute("width")).toBe("150");
  });

  it("renders stage 2 for stage 1", () => {
    const { container } = render(<PetCharacter stage={1} accessories={[]} />);
    const svg = container.querySelector("svg");
    expect(svg).toBeInTheDocument();
    // Stage 2 SVG is 250x250
    expect(svg?.getAttribute("width")).toBe("250");
  });

  it("renders stage 3 for stage 2", () => {
    const { container } = render(<PetCharacter stage={2} accessories={[]} />);
    const svg = container.querySelector("svg");
    expect(svg).toBeInTheDocument();
    // Stage 3 SVG is 350x350
    expect(svg?.getAttribute("width")).toBe("350");
  });

  it("defaults to stage 1 for unknown stage", () => {
    const { container } = render(<PetCharacter stage={99} accessories={[]} />);
    const svg = container.querySelector("svg");
    expect(svg?.getAttribute("width")).toBe("150");
  });

  it("passes accessories to stage component", () => {
    const { container } = render(<PetCharacter stage={0} accessories={["party_hat"]} />);
    // Party hat renders a polygon — check it exists
    const polygons = container.querySelectorAll("polygon");
    expect(polygons.length).toBeGreaterThan(0);
  });
});
