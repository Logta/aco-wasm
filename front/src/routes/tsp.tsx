import { createFileRoute } from "@tanstack/react-router";
import TSPPage from "../pages/TSPPage";

export const Route = createFileRoute("/tsp")({
  component: TSPPage,
});