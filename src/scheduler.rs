/// # Requirements
///
/// - Determine a set of candidate workers on which a task could run
/// - Score the candidate workers from best to worst
/// - Pick the worker with the best score
trait Scheduler {
    fn select_candidate_nodes();
    fn score();
    fn pick();
}
