import { createFileRoute } from "@tanstack/react-router";
import EducationPage from "../pages/EducationPage";

export const Route = createFileRoute("/education")({
  component: EducationPage,
});